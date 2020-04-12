



import java.util.*; 
public class Obj
{ 
public static void  main(String []args) 
{ 
    System.out.println("180410107071         dwij patel"); 
    Random r=new Random(); 
    int a[][]=new int[6][6]; 
    int i,j,count; 

    for(i=0;i<6;i++) //input loop
    { 
        for(j=0;j<6;j++) 
        { 
            a[i][j]=r.nextInt(2); 
        } 
    } 

    for(i=0;i<6;i++)    //output loop
    { 
        for(j=0;j<6;j++) 
        {  
            System.out.print(a[i][j]+" "); 
        } 
        System.out.println(); 
    } 

    System.out.println("Row wise"); 
    for(i=0;i<6;i++)   
       { 
        count=0; 
        for(j=0;j<6;j++) 
        { 
            if(a[i][j]==1) 
            count++; 
        } 

    if(count%2==0) 
        System.out.println("Row "+(i+1)+":even"); 
        else
        System.out.println("Row "+(i+1)+":odd"); 
    }
    System.out.println(); 
    System.out.println("Column wise"); 
    for(i=0;i<6;i++) 
      { 
        count=0; 
        for(j=0;j<6;j++) 
        { 
            if(a[j][i]==1) 
            count++; 
        }
       if(count%2==0) 
        System.out.println("Column "+(i+1)+":even"); 
        else
        System.out.println("Column "+(i+1)+":odd"); 
    } 
}

}




