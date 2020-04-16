



import java.util.Random;

class c2
{
	public static void main(String []args)
	{
		System.out.println("180410107028     harsh gandhi");

		
		Random r1=new Random();
		
		int i;
		for(i=0;i<=9;i++)
		{
			String str=new String();
			char ch;
			int n;
			int count=0;
			while(count<3)
			{
				n=r1.nextInt(26)+65;
				ch=(char)n;
				str=str+ch;
				count++;
			}
			int a;
			a=r1.nextInt(9000)+1000;
			System.out.println(str+" "+a);
		}
	}
}



