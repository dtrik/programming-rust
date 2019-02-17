extern crate iron;
#[macro_use] extern crate mime; //#[macro_use] attribute alerts that macros
                                //exported are being used

use iron::prelude::*;   //all public names of iron::prelude module are visible
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000 ...");
    //create a server and listens on TCP Port 3000
    //server uses get_form  function to handle requests
    Iron::new(get_form).http("localhost:3000").unwrap();
}

//_ prefix indicates variable is expected to be unused
//function uses mutable reference to Request
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="m"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}

