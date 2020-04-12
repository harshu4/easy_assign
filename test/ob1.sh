echo "target/debug/get_output dwij patel 180410107071"
echo "Practical-7"   
echo "Enter a number"   
read num   
echo $num
fact=1   
for((i=2;i<=num;i++))   
{   
fact=$((fact * i))   
}   
echo $fact   