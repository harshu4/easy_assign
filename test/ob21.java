




import java.util.Scanner; 
 
public class VA
{  public static void main(String[] args)  
 { 
  System.out.println("180410107007"); 
  System.out.println("dwij patel");
   
  Scanner input = new Scanner(System.in); 
 
  System.out.print("Enter five integers: ");   int[] list = new int[5];  
  for (int i = 0; i < list.length; i++)  
  { 
   list[i] = input.nextInt(); 
  } 
 
  System.out.println("The smallest element is " + min(list)); 
 } 
 
 
 public static int min(int[] list)  
 { 
  int min = list[list.length - 1];   int index = list.length - 1;   return min(list, index, min); 
 } 
 
 private static int min(int[] list, int index, int min)  
 {   if (index < 0) 
  {    return min;   }   else if (list[index] < min)  
  {     return min(list, index - 1, list[index]); 
  }   else   {    return min(list, index - 1, min); 
  }  } 
}   





