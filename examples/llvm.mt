let name = "llvm";
let version = [19,1,3];
download("LLVM", name + "-project-19.1.3", "tar.xz", "https://github.com/llvm/llvm-project/releases/download/llvmorg-19.1.3/llvm-project-19.1.3.src.tar.xz", "324d483ff0b714c8ce7819a1b679dd9e4706cf91c6caf7336dc4ac0c1d3bf636");
step("Extract LLVM", "tar", "-xvf " + name + "-project-19.1.3.tar.xz");
step("CD into LLVM", "cd", name + "-project-19.1.3.src");
step("Configure LLVM", "cmake", "-DLLVM_ENABLE_PROJECTS=clang;llvm -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles" ./"+name);
step("Build LLVM", "make", "");
step("Install LLVM", "make", "install", "DESTDIR=../pkg");
install(name);