let name = "binutils-xcompiled";
let version = [2,43];
download_extract("Binutils", "binutils-2.43", ".tar.gz", "https://ftp.gnu.org/gnu/binutils/binutils-2.43.tar.gz", "025c436d15049076ebe511d29651cc4785ee502965a8839936a65518582bdd64");
step("Create tools directory", "mkdir", "../pkg/tools");
step("Configure Binutils", "./configure", "--prefix=/tmp/pkg \
                                           --with-sysroot=/tmp/pkg/tools \
                                           --target=x86_64-unknown-linux-musl \
                                           --disable-nls \
                                           --enable-gprofng=no \
                                           --disable-werror \
                                           --enable-new-dtags \
                                           --enable-default-hash-style=gnu");
step("Build", "make", "");
step("Install", "make", "install");
install(name);