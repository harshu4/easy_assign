echo "target/debug/get_output dwij patel 180410107071"
echo "Practical-9"     
echo "Enter the three subject marks for the student"   
read m1 m2 m3     
echo $m1
echo $m2
echo $m3
sum1=`expr $m1 + $m2 + $m3`   
echo "Sum of 3 subjects are:" $sum1   
per=`expr $sum1 / 3`   
echo "Percentage:" $per   
if [ $per -ge 60 ]   
then   
echo "You get Distinction"   
elif [ $per -ge 50 ]   
then   
echo "You get First class"   
elif [ $per -ge    40 ]   
then   
echo "You get Second class"   
else   
      echo "You get Fail"   
fi 