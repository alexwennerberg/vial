use {std::fs::File, vial::Request};

#[test]
fn basic_req_methods() {
    let req = Request::from_reader(File::open("tests/http/another_GET.txt").unwrap()).unwrap();
    assert_eq!("GET", req.method());
    assert_eq!("/docs/index.html", req.path());
    assert_eq!("/docs/index.html?v=22", req.full_path());
    assert!(req.has_query("v"));
    assert!(!req.has_query("xxx"));
    assert_eq!(Some("22"), req.query("v"));
    assert_eq!("", req.body());
    assert_eq!("gzip, deflate", req.header("accept-encoding").unwrap());
    assert_eq!(
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)",
        req.header("User-Agent").unwrap()
    );
}

#[test]
#[cfg(feature = "json_serde")]
fn json() {
    use nanoserde::DeJson;
    let req = Request::from_reader(File::open("tests/http/json_POST.txt").unwrap()).unwrap();
    #[derive(DeJson)]
    struct Sample {
        hello: String,
    }

    assert_eq!("world".to_string(), req.json::<Sample>().unwrap().hello);
    let req = Request::from_reader(File::open("tests/http/bad_json_POST.txt").unwrap()).unwrap();
    assert!(req.json::<Sample>().is_err());
}
