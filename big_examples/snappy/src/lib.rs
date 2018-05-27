#[macro_use]
extern crate cwa;

extern crate snap;

main!({
    ::cwa::io::with_stdin(|stdin| {
        ::cwa::io::with_stdout(|stdout| {
            
            let mut writer = snap::Writer::new(stdout);
            ::std::io::copy(stdin, &mut writer).unwrap();
        });
    });
});
