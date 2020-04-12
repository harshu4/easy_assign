



import java.util.Random;
class VA
{
public static void main(String[] args)
{
System.out.println("180410107007"); 
System.out.println("dwij patel");
Random rand = new Random(1000);
for(int i = 0; i < 100; i++)
{
System.out.format("%3d",rand.nextInt(49));
if((i+1)%20==0)
{
System.out.println();
}
}
}
}



