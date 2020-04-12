# easy_assign

Takes screenshots of your bash program and generates a pdf for you , easy to use, syntax are not idiomatic so don't even bother :P

The ob.txt file contains list of the program you want to run and take screenshot , seprate them with ---!--- . now run the program as
follows

written in rust for speed and efficiency !
How to make it run

```
git clone https://github.com/harshu4/easy_assign.git
cd easy_assign
chmod +x get_output
./get_output {First_Name} {Last_Name} {En_No}
./get_output {no of programs}
```


Further Upgrades  [#TODO]
- This code currently only supports bash , future updates may include support for java and other languages
- Themes to select from , currently only default format is supported 
- Adding comments and making code more readable 


Highly unstable may crash , deals with xterm losely may leave terminal open !

You may check test_working.pdf for sample
