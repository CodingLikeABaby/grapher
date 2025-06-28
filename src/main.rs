use clap::Parser;
use base64::{engine::general_purpose, Engine};
use url::form_urlencoded;
use arboard::Clipboard;

#[derive(Parser, Debug)]
#[command(author, version, about = "Mini outil encodeur multi-format")]
struct Cli {
    /// Base64 encode
    #[arg(short = 'B', long = "Base64")]
    base64: bool,

    /// Base64 decode
    #[arg(short = 'b', long = "base64")]
    base64decode: bool,

    /// URL encode
    #[arg(short = 'U', long = "Url")]
    url: bool,

    /// URL decode
    #[arg(short = 'u', long = "url")]
    urldecode: bool,

    /// Duble encoding (compatible with -B, -b, -U, -u)
    #[arg(short = 'd', long = "dub")]
    dub: bool,

    /// Add result to clipboard
    #[arg(short = 'c', long = "copy")]
    cop: bool,

    /// Base64 URL encode (cannot be used with -U, -u, -B, -b)
    #[arg(short = 'M', long = "b2u")]
    b2u: bool,

    /// Base64 URL decode (cannot be used with -U, -u, -B, -b)
    #[arg(short = 'm', long = "b2ud")]
    b2ud: bool,

    /// Text to process
    input: String,
}

fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}


fn decode_base64(input: &str) -> Result<String, String> {
    match general_purpose::STANDARD.decode(input) {
        Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
            Ok(text) => Ok(text),
            Err(_) => Err(String::from("⚠️ invalid UTF-8")),
        },
        Err(_) => Err(String::from("❌ base64 error")),
    }
}


fn encode_url(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}


fn decode_url(input: &str) -> String {
    let decoded: String = form_urlencoded::parse(input.as_bytes())
        .map(|(k, _)| k.to_string())
        .collect::<Vec<String>>()
        .join("");
    decoded 
}



fn base64_and_url(input: &str) -> String {
    let first_encode = encode_base64(input);
    let second_encode = encode_url(&first_encode);
    second_encode
}


fn base64_and_url_decode(input: &str) -> Result<String, String> {
    let first_decode = decode_url(input);
    let second_decode = decode_base64(&first_decode);
    second_decode
}



fn copy_to_clipboard(input: &str) {
    let mut clipboard = Clipboard::new().expect("Error while accessing clipboard");
    clipboard.set_text(input.to_string()).expect("Error while copying text");

}






// Doubles encoding


fn dub_base64_encode(input: &str) -> String {
    let encoded = encode_base64(input);
    let dubencoded = encode_base64(&encoded);
    dubencoded
}

fn dub_base64_decode(input: &str) -> Result<String, String> {
    match decode_base64(input) {
        Ok(decoded_once) => {
            match decode_base64(&decoded_once) {
                Ok(decoded_twice) => Ok(decoded_twice),          
                Err(_) => Ok(decoded_once),                      
            }
        }
        Err(e) => Err(e),
    }
    
}



fn dub_url_encode(input: &str) -> String {
    let encoded = encode_url(input);
    let dubencoded = encode_url(&encoded);
    dubencoded
}


fn dub_url_decode(input: &str) -> String {
    let decoded = decode_url(input);
    let dubdecoded = decode_url(&decoded);
    dubdecoded
}




fn main() {
    let args = Cli::parse();



    // Check conditions that we don't want to be combined
    if (args.b2u && args.base64) || (args.b2u && args.url) || (args.b2ud && args.base64) || (args.b2ud && args.url) {
        eprintln!("⚠️ Base64 URL encoding cannot be combined with Base64 or URL encoding alone at the same time, please use them separately");
        return;
    } 
    else if (args.b2u && args.base64decode) || (args.b2u && args.urldecode) || (args.b2ud && args.base64decode) || (args.b2ud && args.urldecode) {
        eprintln!("⚠️ Base64 URL decoding cannot be combined with Bsse64 or URL decoding alone at the same time, please use them separately");
        return;
    }




    // Check that there is no opposite arguments in the same request
    if args.base64 && args.base64decode {
        eprintln!("⚠️ These arguments are contradictory");
        return;
    }
    else if args.url && args.urldecode {
        eprintln!("⚠️ These arguments are contradictory");
        return;
    }
    else if args.b2u && args.b2ud {
        eprintln!("⚠️ These arguments are contradictory");
        return;
    }



    // Return an error if we use -d with -M or -m
    if args.b2u && args.dub {
        eprintln!("⚠️ You can't use the Duble function with Base64 URL encoding");
        return;
    }
    else if args.b2ud && args.dub {
        eprintln!("⚠️ You can't use the Duble function with Base64 URL decoding");
        return;
    }



    // Say to use -M or -m instead of -B and -U or -b and -u
    if args.base64 && args.url {
        eprintln!("⚠️ You must use Base64 URL function instead of Base64 and URL separately");
        return;
    }
    else if args.base64decode && args.urldecode {
        eprintln!("⚠️ You must use the Base64 URL function instead of Base64 and URL separately");
        return;
    }


    // Return an error if the user use -B and -u or -U and -b at the same time
    if args.base64 && args.urldecode {
        eprintln!("⚠️ -B and -u cannot be combined");
        return;
    }
    else if args.base64decode && args.url {
        eprintln!("⚠️ -b and -U cannot be combined");
        return;
    }




    // base64 encode
    if args.base64 && !args.dub {
        let res = encode_base64(&args.input);
        println!("Encoded base64 : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }



    // Dub base64 encode
    if args.base64 && args.dub {
        let res = dub_base64_encode(&args.input);
        println!("Dub base64 encoded : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }

    // base64 decode
    if args.base64decode && !args.dub {
        match decode_base64(&args.input) {
        Ok(decoded) => {
            println!("Decoded base64 : {}", decoded);
            if args.cop {  // Copy to clipboard
                copy_to_clipboard(&decoded);
                println!("Text copied to clipboard");
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    }


    // Dub base64 decode
    if args.base64decode && args.dub {
        match dub_base64_decode(&args.input) {
        Ok(text) => {
            println!("Dub base64 decoded: {}", text);
            if args.cop {  // Copy to clipboard
                copy_to_clipboard(&text);
                println!("Text copied to clipboard");
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    }




    // url encode
    if args.url && !args.dub {  
        let res = encode_url(&args.input);
        println!("Encoded URL : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }


    // Dub url encode
    if args.url && args.dub {
        let res = dub_url_encode(&args.input);
        println!("Dub URL encoded : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }


    // url decode
    if args.urldecode && !args.dub {
        let res = decode_url(&args.input);
        println!("Decoded URL : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }


    // Dub url decode
    if args.urldecode && args.dub {
        let res = dub_url_decode(&args.input);
        println!("Dub URl decoded : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }
    


    // Base64 and URL encode
    if args.b2u {
        let res = base64_and_url(&args.input);
        println!("Base64 URL encoded : {}", res);
        // Copy to clipboard
        if args.cop {
            copy_to_clipboard(&res);
            println!("Text copied to clipboard");
        }
    }


    // URL and Base64 decode
    if args.b2ud {
        match base64_and_url_decode(&args.input) {
        Ok(text) => {
            println!("URL Base64 decode : {}", text);
            if args.cop {  // Copy to clipboard
                copy_to_clipboard(&text);
                println!("Text copied to clipboard");
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    }



    // Return an error if there is no encoding or decoding arguments
    if !args.base64 && !args.base64decode && !args.url && !args.urldecode && !args.b2u && !args.b2ud {
        eprintln!("⚠️ No selected mod. Use --help to see options");
    }


    
}

