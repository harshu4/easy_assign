




import java.io.*; 
import java.security.InvalidParameterException; import java.util.Arrays; import java.util.HashSet; import java.util.TreeSet; import java.util.Collections; import java.util.Iterator;  
 
public class VA
{ 
 public static void main(String[] args) throws FileNotFoundException  
 { 
  System.out.println("180410107007"); 
  System.out.println("harsh gandhi");

       
 if (args.length != 1) 
            throw new InvalidParameterException("Usage: fullFilePathName"); 
 
        File file = new File(args[0]); 
         
 if (!file.isFile()) 
            throw new FileNotFoundException(file + " is not a file."); 
 
        try (BufferedReader in = new BufferedReader(new InputStreamReader(new FileInputStream(file)), 10000)) 
  { 
  String inputS; 
  StringBuilder sb = new StringBuilder(10000);              while ((inputS = in.readLine()) != null)                  sb.append(inputS); 
 
             String[] words = sb.toString().split("\\s+"); 
      
             TreeSet<String> ndWords = new TreeSet<>(Arrays.asList(words)); 
   
  Iterator<String> itr = ndWords.descendingIterator();    String s; 
  while (itr.hasNext())  
         {  
              s = itr.next();  
       System.out.println(s); 
         }  
 } 
 catch (IOException e)  
 { 
            e.printStackTrace(); 
            System.exit(0); 
        }
}  
 } 



