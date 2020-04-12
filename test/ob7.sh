echo "target/debug/get_output test java harsh gandhi 180410107028"
echo "Practical 10"   
usrdate=$1   
date -d "$usrdate"    > /dev/null 2>&1   
if [ $? -eq 0 ]; then   
         echo "Date $usrdate was valid"   
else   
         echo "Date $usrdate was invalid"   
fi   