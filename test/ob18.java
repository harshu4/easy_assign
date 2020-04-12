

import javafx.application.Application;  
import javafx.event.ActionEvent;  
import javafx.event.EventHandler;  
import javafx.stage.Stage;  
import javafx.scene.Group;  
import javafx.scene.Scene;  
import javafx.scene.paint.Color;  
import javafx.scene.shape.Circle;  
public class ob18 extends Application {  
@Override  
public void start(Stage primaryStage) {  
primaryStage.setTitle("harsh gandhi-180410107028");  
Circle c1=new Circle();  
c1.setCenterX(250);  
c1.setCenterY(250);  
c1.setRadius(100);  
c1.setOnMousePressed(e -> {  
c1.setFill(Color.RED);  
});  
 
c1.setOnMouseReleased(e -> { 
c1.setFill(Color.BLUE);  
});  
Group g1=new Group(c1); 
Scene s1=new Scene(g1,500,500); 
primaryStage.setScene(s1);  
primaryStage.show();  
}  
  public static void main(String[] args) {  
launch(args);   
}  
} 


