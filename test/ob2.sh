echo "target/debug/get_output"
echo "Practical-7"   
echo "Enter the number"   
read a   
echo $a
i=2   
z=0   
echo $a   
while [ $i -lt $a ]   
do   
s=`expr $a % $i`   
if [ $s -eq $z ]   
then   
echo "Not Prime"   
exit   
else   
i=`expr $i + 1`   
fi   
done   
echo "Prime number"   