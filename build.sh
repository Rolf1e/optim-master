#TODO fix


echo "Building project"

cargo build --release
cp target/release/libknapsack.so knapsack
cp target/release/tp1 r-tp1
mv knapsack/libknapsack.so knapsack/knapsack.so

echo "Project builded"

#python3.7 knapsack/__init__.py
