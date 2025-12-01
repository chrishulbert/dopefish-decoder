help:
	cat Makefile

4:
	RUST_BACKTRACE=1 cargo run data/keen4/keen4.exe data/keen4/egagraph.ck4 data/keen4/gamemaps.ck4

5:
	RUST_BACKTRACE=1 cargo run data/keen5/keen5.exe data/keen5/egagraph.ck5 data/keen5/gamemaps.ck5

6:
	RUST_BACKTRACE=1 cargo run data/keen6/keen6.exe data/keen6/egagraph.ck6 data/keen6/gamemaps.ck5

clean:
	rm -f Output*

c4: clean 4

c5: clean 5

c6: clean 6

test:
	cargo test
