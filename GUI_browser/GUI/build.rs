fn main() {
    let dst = cmake::Config::new("..").build();

    // ビルドしたライブラリのあるディレクトリをRustに伝える
    println!("cargo:rustc-link-search=native={}", dst.display());

    // 例えば glad, glfw の静的ライブラリをリンク
    println!("cargo:rustc-link-lib=static=glad");
    println!("cargo:rustc-link-lib=static=glfw");

    // C++標準ライブラリもリンク（環境によって名前は変わることがある）
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

