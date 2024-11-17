let name = "musl-xcompiled";
let version = [1,2,5];
download_extract("MUSL", "musl-1.2.5", ".tar.gz", "https://musl.libc.org/releases/musl-1.2.5.tar.gz", "a9a118bbe84d8764da0ea0d28b3ab3fae8477fc7e4085d90102b8596fc7c75e4");
set_env("CC", "clang -static");
step("Configure", "./configure",  "--prefix=../pkg");
step("Build", "make", "");
step("Install", "make", "install DESTDIR=../pkg TARGET=x86_64-linux-musl");
install(name);
unset_env("CC");