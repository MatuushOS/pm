#!command -v pm
let name = "hello";
let version = [2, 12];
download_extract("GNU Hello",
"hello-2.12",
".tar.gz",
"https://ftp.gnu.org/gnu/hello/hello-2.12.tar.gz",
"cf04af86dc085268c5f4470fbae49b18afbc221b78096aab842d934a76bad0ab");
step("Configure the package", "./configure", "");
step("Clean", "make", "clean");
step("Build", "make", "");
step("Install", "make", "install DESTDIR=../pkg");
mkpackage(name);
