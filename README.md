
Use nightly tool-chain <= 1.48

required :

 `pip install matplotlib`

To execute project : 

Simply execute build file :D 

Or .. 
By yourself

 1. build Rust sources
 `cargo build --release`

 2. copy and rename libknapsack.so into knapsack.so

 `
 	cp target/release/libknapsack.so knapsack
  	mv knapsack/libknapsack.so knapsack/knapsack.so
 `

 3. run python file with some data 
 ` python3.7 knapsack/__init__.py`



