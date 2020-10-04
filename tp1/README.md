Implementation of : 
https://www-lisic.univ-littoral.fr/~verel/TEACHING/20-21/optimisation-M1app/tp01.pdf

Use nightly tool-chain <= 1.48

required :

 `pip install matplotlib`

To execute project : 

 1. build Rust sources
 `cargo build --release`

 2. copy and rename libknapsack.so into knapsack.so

 `
 	cp target/release/libknapsack.so knapsack
  	mv knapsack/libknapsack.so knapsack/knapsack.so
 `

 3. run python file with some data 
 ` python3.7 knapsack/__init__.py`


