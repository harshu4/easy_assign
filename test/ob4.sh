echo "target/debug/get_output"
echo "Practical-8"     
echo "Enter a number"   
read N   
echo $N
a=0     
b=1       
echo "The Fibonacci series is : "   
for (( i=0; i<N; i++ ))     
do   
     echo -n "$a "   
     fn=$((a + b))     
     a=$b     
     b=$fn     
done 