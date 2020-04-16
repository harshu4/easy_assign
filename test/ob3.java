



import java.util.Scanner;
class VA
{
	public static void main(String []args)
	{
		System.out.println("180410107028     harsh gandhi");
		Scanner s=new Scanner(System.in);
		double weight,height,bmi;
		System.out.println("Enter height in inches");
		height=s.nextDouble();
		height=height/0.0254;
		System.out.println("Enter weight in pounds");
		weight=s.nextDouble();
		weight=weight*0.45359237;
		bmi=weight/(height*height);
		System.out.println("BMI="+bmi);
	}
}


