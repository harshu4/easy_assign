


import java.util.Scanner;
class b2
{
	public static void main(String []args)
	{
		System.out.println("180410107028     harsh gandhi");
		
		Scanner sc=new Scanner(System.in);
		char ch;

		String s1=new String();
	
		System.out.println("Enter String");

		s1=sc.nextLine();
 
		int l=s1.length();

		
		int v=0,c,i;
		for(i=0;i<l;i++)
		{
			ch=s1.charAt(i);
			if(ch=='a' || ch=='e' ||ch=='i' || ch=='o' ||ch=='u' )
			{
				v++;	
			}	
		}
		c=l-v;		

		System.out.println("No of vowel="+v);

		System.out.println("No of consonant="+c);
	}
}



