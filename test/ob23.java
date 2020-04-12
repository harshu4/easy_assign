


import java.util.PriorityQueue; 
 
public class VA 
{ 
     public static void main(String[] args)  
 { 
  System.out.println("180410107007"); 
  System.out.println("dwij patel");


  MyPriorityQueue<String> queue = new MyPriorityQueue<>();          queue.offer("1");          queue.offer("2");          queue.offer("3"); 
 
         MyPriorityQueue<String> queue1 = null;          try  
  { 
              queue1 = (MyPriorityQueue<String>)(queue.clone()); 
         }  
  catch (CloneNotSupportedException e)  
  { 
              e.printStackTrace(); 
         } 
         System.out.print(queue1); 
     } 
 
     static class MyPriorityQueue<E> extends PriorityQueue<E> implements Cloneable  
 { 
         @Override 
         public Object clone() throws CloneNotSupportedException  
  { 
   MyPriorityQueue<E> clone = new MyPriorityQueue<>(); 
              this.forEach(clone::offer);               return clone; 
         } 
 
     } 



