all:
	rm ./libcpptest.so | true
	g++ -shared -o libcpptest.so -fPIC cpptest.cpp
	rm librslibtest.a | true
	cd rslib  && cargo build --release 
	mv ./rslib/target/release/deps/librslibtest.a .
	rm cpptest.o cpptest.a | true
	g++ -c -fPIC cpptest.cpp -o cpptest.o && ar rvs libcpptest.a cpptest.o
	rm test*o | true
	echo "${LIBRARY_PATH} ${LD_LIBRARY_PATH}"
	rm test | true
	rustc --test ./testlibs.rs -ldl -lpthread -lgcc_s -lc -lm -lrt -lstdc++ -L.
	./testlibs --nocapture

