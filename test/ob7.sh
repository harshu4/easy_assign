echo "target/debug/get_output dwij patel 180410107071"
echo "Practical 10"   
usrdate=$1   
date -d "$usrdate"    > /dev/null 2>&1   
if [ $? -eq 0 ]; then   
         echo "Date $usrdate was valid"   
else   
         echo "Date $usrdate was invalid"   
fi   