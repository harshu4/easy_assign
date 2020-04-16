



import java.util.Scanner;
class a2
{
	public static void main(String []args)
	{
		System.out.println("180410107028     harsh gandhi");
		Scanner s=new Scanner(System.in);
		int a[]=new int[3];
		int i,j,temp;
		for(i=0;i<3;i++)
		{
			System.out.println("Enter no"+(i+1));
			a[i]=s.nextInt();
		}
		for(i=0;i<3;i++)
		{
			for(j=0;j<3;j++)
			{
				if(a[i]>a[j])
				{
					temp=a[i];
					a[i]=a[j];
					a[j]=temp;
				}
			}
		}
		System.out.println();
		for(i=0;i<3;i++)
		{
			System.out.println(a[i]);
		}
	}
}



