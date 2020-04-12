echo "target/debug/get_output dwij patel 180410107071"
read a     
echo $a
read b     
echo $b
echo "Enter Choice :"   
echo "1. Addition"   
echo "2. Subtraction"   
echo "3. Multiplication"   
echo "4. Division"   
read ch     
echo $ch
case $ch in   
1)res=`echo $a + $b | bc`     
;;     
2)res=`echo $a - $b | bc`     
;;     
3)res=`echo $a \* $b | bc`     
;;     
4)res=`echo "scale=2; $a / $b" | bc`     
;;     
esac   
echo "Result : $res" 