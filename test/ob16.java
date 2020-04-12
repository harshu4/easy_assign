



 import java.io.FileInputStream; 
import java.io.FileNotFoundException; 
 import javafx.application.Application;  
import javafx.stage.Stage;  
import javafx.scene.Scene;  
import javafx.scene.image.Image;  
import javafx.scene.image.ImageView;  
import javafx.scene.layout.GridPane;  
  public class ob16 extends Application {  
@Override  
public void start(Stage primaryStage) throws Exception {  
GridPane p1=new GridPane();  
for (int i = 0; i < 3; i++) {  
for (int j = 0; j < 3; j++) {  
int n = (int)(Math.random() * 3);  
if (n == 0)  
p1.add(new ImageView(new Image(new FileInputStream("aa.png"))), i, j);  
else  
p1.add(new ImageView(new Image(new FileInputStream("aa.png"))), i, j);  
}  
}  
 Scene scene = new Scene(p1, 700, 700);  
primaryStage.setTitle("dwij patel-180410107071");   
primaryStage.setScene(scene);   
primaryStage.show();   
}  
public static void main(String[] args) {  
launch(args);  
}  
} 

