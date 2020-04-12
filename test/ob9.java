



import java.util.Scanner;
class b3
{
	static int a[]=new int[10];
	static int b[]=new int[10];
	public static void main(String []args)
	{
		System.out.println("180410107028     harsh gandhi");
		Scanner s=new Scanner(System.in);
		System.out.println("Enter 10 numbers");
		int i,temp;
		for(i=0;i<10;i++)
		{
			a[i]=s.nextInt();
		}
		System.out.println("Reverse numbers");
		Rev();
		Display();
	}
	static void Rev()
	{
		int i,j=0;
		for(i=9;i>=0;i--)
		{
			b[j]=a[i];
			j++;
		}
	}
	static void Display()
	{
		int i;
		for(i=0;i<10;i++)
		{
			System.out.println("b["+i+"]="+b[i]);
		}
	}
}



