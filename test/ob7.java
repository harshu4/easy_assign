



import java.util.Scanner;
class d2
{
	public static void main(String []args)
	{
	System.out.println("180410107071     dwij patel");
		Scanner s1=new Scanner(System.in);
		int i=2,a,n;
		a=s1.nextInt();
		n=a;
		while(i<=n/2)
		{
			if(a%i==0)
			{
				a=a/i;
				System.out.println(i);
			}
			else
			{
				i++;
			}
		}
	}
}


