with import <nixpkgs> {};

let
  mylibedit = stdenv.lib.overrideDerivation libedit (drv: {
    prePatch = ''
      substituteInPlace configure --replace 0:53:0 2:0:0
    '';
  });

  rust = stdenv.mkDerivation {
    name = "rustc-1.4.0-nightly";
    src = fetchurl {
      url = "https://static.rust-lang.org/dist/rust-nightly-x86_64-apple-darwin.tar.gz";
      sha256 = "1pdsi3qfkxsxappq1l5yswpwhzickm6mrhhpxa6yzw3vzfjcx9sl";
    };

    installPhase = ''
      ./install.sh --prefix=$out --verbose
    '';

    fixupPhase = ''
      for file in $(find $out -name '*.dylib') $out/bin/{rustc,rustdoc,cargo}; do
        install_name_tool -change /usr/lib/libedit.3.dylib ${mylibedit}/lib/libedit.dylib $file
        for dylib in $(otool -L $file | grep x86_64-apple-darwin | cut -d' ' -f1); do
          install_name_tool -id $file $file
          install_name_tool -change $dylib "$(echo $dylib | sed s,x86_64-apple-darwin/stage./lib/rustlib/x86_64-apple-darwin,$out,)" $file
        done
      done

      install_name_tool \
        -change /usr/lib/libcurl.4.dylib ${curl}/lib/libcurl.dylib \
        $out/bin/cargo
    '';
  };

in stdenv.mkDerivation {
  name = "jude-web";
  buildInputs = [ rust openssl ];
}
