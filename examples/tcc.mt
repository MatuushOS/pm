let name = "tcc";
let version = [0, 9, 27];
download_extract("TCC",
"tcc-0.9.27",
".tar.bz2",
"http://download.savannah.gnu.org/releases/tinycc/tcc-0.9.27.tar.bz2",
"de23af78fca90ce32dff2dd45b3432b2334740bb9bb7b05bf60fdbfc396ceb9c");
step("Configure TCC", "./configure", "");
step("Clean", "make", "clean");
step("Build TCC", "make", "");
step("Install TCC", "make", "install DESTDIR=../pkg");
mkpackage(name);