// STDIN BASED STUB

let option: Option<char> = stdin().bytes()
.next()
.and_then(|result| result.ok())
.map(|byte| byte as char)
.filter(|c| c.is_alphabetic())
.map(|mchar| mchar.to_lowercase().next().unwrap());

match option {
    Some('y')   => {

    }
    Some('n')   => {

    }
    Some(other)  => {

    }
    None    => {

    }
}
