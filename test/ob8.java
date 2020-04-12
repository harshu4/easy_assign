





import java.util.Scanner;
class a3
{
	public static void main(String []args)
	{
		System.out.println("180410107071     dwij patel");
		Scanner s1=new Scanner(System.in);
		int a,b,c;
		a=s1.nextInt();
		b=s1.nextInt();		
		c=Gcd1(a,b);
		System.out.println("Gcd="+c);
	}
	static int Gcd1(int x,int y)
	{
		int i=1;
		int d=0;
		while(i<=x)
		{
			if(x%i==0 && y%i==0)
			{
				d=i;
			}
			i++;
		}
		return d;
	}
}



