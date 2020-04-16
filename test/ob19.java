

 
import javafx.application.Application;  
import javafx.event.ActionEvent;  
import javafx.event.EventHandler;  
import javafx.stage.Stage;  
import javafx.scene.Group;  
import javafx.scene.Scene;  
import javafx.scene.control.Button;  
import javafx.scene.control.RadioButton;  
import javafx.scene.control.ToggleGroup;  
import javafx.scene.paint.Color;  
import javafx.scene.text.Text;  
public class ob19 extends Application {  
@Override  
public void start(Stage primaryStage) {  
primaryStage.setTitle("harsh gandhi-180410107028");  
Button b1=new Button("Left");  
b1.setLayoutX(100);  
b1.setLayoutY(400);  
Button b2=new Button("Right");  
b2.setLayoutX(400);  
b2.setLayoutY(400);  
  String str1="HELLO WORLD..!!"; 
Text t1=new Text();  
t1.setText(str1);  
t1.setX(200);  
t1.setY(150);  
  
EventHandler<ActionEvent> event = new EventHandler<ActionEvent>() { public void handle(ActionEvent e)  
{  
double x1=t1.getX()-10.0;  
if(x1>10) {  
t1.setText(str1);  
t1.setX(x1);  
}  
else  
{  
t1.setText("Press Right Button");  
}  
}  
};  
  b1.setOnAction(event);  
EventHandler<ActionEvent> event1 = new EventHandler<ActionEvent>() {  
public void handle(ActionEvent e)  
{  
double x1=t1.getX()+10.0;  
if(x1<420)  
{  
t1.setText(str1);  
double x; 
t1.setX(x1); 
}  
else  
{  
t1.setText("Press Left Button");  
}  
}  
};  
  // when button is Pressed  
  b2.setOnAction(event1);  
ToggleGroup gColor = new ToggleGroup();  
RadioButton r1=new RadioButton("Red");  
r1.setLayoutX(150);  
r1.setLayoutY(450); 
  RadioButton r2=new RadioButton("Blue");  
r2.setLayoutX(250);  
r2.setLayoutY(450);  
  RadioButton r3=new RadioButton("Green");  
r3.setLayoutX(350);  
r3.setLayoutY(450);  
  r1.setToggleGroup(gColor);  
r2.setToggleGroup(gColor);  
r3.setToggleGroup(gColor);  
  EventHandler<ActionEvent> event3 = new EventHandler<ActionEvent>()  
{  
public void handle(ActionEvent e)  
{ 
if(r1.isSelected())  
{  
t1.setFill(Color.RED);  
}  
} 
}; 
  r1.setOnAction(event3);  
EventHandler<ActionEvent> event4 = new EventHandler<ActionEvent>() {  
public void handle(ActionEvent e)  
{  
if(r2.isSelected())  
{  
t1.setFill(Color.BLUE);  
}  
} 
};  
  r2.setOnAction(event4);  
EventHandler<ActionEvent> event5 = new EventHandler<ActionEvent>() {  
public void handle(ActionEvent e)  
{  
if(r3.isSelected())  
{  
t1.setFill(Color.GREEN);  
}  
} 
};  
  r3.setOnAction(event5);  
Group g1=new Group(b1,b2,t1,r1,r2,r3);  
Scene s1=new Scene(g1,500,500);  
primaryStage.setScene(s1);  
primaryStage.show();}  
public static void main(String[] args) {  
launch(args);  
} 
} 



