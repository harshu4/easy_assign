echo "target/debug/get_output Harsh Gandhi 180410107028"
echo "Practical-8"   
echo -n "Enter number : "   
read n   
echo $n
sd=0   
rev=""   
on=$n   
echo $n   
while [ $n -gt 0 ]   
do   
    sd=$(( $n % 10 )) # get Remainder     
     n=$(( $n / 10 ))    # get next digit   
     # store previous number and current digit in reverse     
     rev=$( echo ${rev}${sd} )     
done   
if [ $on -eq $rev ];   
then   
   echo "Number is palindrome"   
else   
   echo "Number is NOT palindrome"   
fi   