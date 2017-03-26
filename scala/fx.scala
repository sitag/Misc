import javafx.application.Application
import javafx.scene.Scene
import javafx.scene.layout.{StackPane, GridPane, Region, BackgroundFill, Background}
import javafx.geometry.Insets;
import javafx.stage.Stage
import javafx.scene.control.Label
import javafx.scene.control.{TextField => JTextField, TextArea => JTextArea}
import javafx.scene.text.Font
import javafx.event.ActionEvent
import javafx.event.EventHandler

import scalafx.scene.control._


package fx {

object Utils {
  import javafx.scene.paint.Color
  import javafx.scene.layout.CornerRadii
  def setBackground(area:JTextArea){
    val region = area.lookup( ".text-area" ).asInstanceOf[Region]
    val style = """|.text-input {
                   |  -font-family:sans-serif
                   |  -fx-background-color: linear-gradient(to bottom, derive(-fx-text-box-border, -10%), -fx-text-box-border),
                   |  linear-gradient(from 0px 0px to 0px 5px, derive(-fx-control-inner-background, -9%), -fx-control-inner-background);
                   |  -fx-control-inner-background: #727272 }""".stripMargin
    area.setStyle(style);
  }
}

object DefaultLayout {
  def background = {
    val insets = new Insets(1, 1, 1, 1)
    new javafx.scene.layout.Background(
      new javafx.scene.layout.BackgroundFill(
        javafx.scene.paint.Color.rgb(72,72,72) , 
        javafx.scene.layout.CornerRadii.EMPTY,
        insets))
  }

  def gridPane() = {
    val grid = new GridPane();
    grid.setHgap(5)
    grid.setVgap(5)
    grid.setPadding(new Insets(3, 3, 3, 3))
    grid.setGridLinesVisible(false)
    grid
  }
  
  def textArea(content:String="", tyle:String="") = {
    val text = new TextArea {
      editable = false
      focusTraversable = false
      opacity = 0.5
      text = content
    }
    text
  }

  def editableTextArea(content:String="", style:String="") = {
    val text = new TextArea {
      editable = true
      focusTraversable = false
      opacity = 0.9
      text = content
    }
    if(style == ""){
      // set font?
    }
    text
  }
}

class Layout {
    val console = Array(DefaultLayout.textArea(), DefaultLayout.textArea())
    val textField = new TextField {
      text = "."
      opacity = 0.8;
      onAction = new EventHandler[ActionEvent] {
        override def handle(event: ActionEvent) {
          val str = text()
          console(1).text = "#" + str
          text() = "."
          }
        }
    }
    val textArea = Array(DefaultLayout.editableTextArea("0"), DefaultLayout.editableTextArea("1"))
    val gridPane = DefaultLayout.gridPane()
    gridPane.setBackground(DefaultLayout.background)
    gridPane.add(textField.delegate, 0, 0)
    gridPane.add(console(0).delegate, 0, 2)
    gridPane.add(console(1).delegate, 1, 2)
    gridPane.add(textArea(0).delegate, 1, 0)
    gridPane.add(textArea(1).delegate, 1, 1)
    Utils.setBackground(textArea(1).delegate)
}


class Shell extends Application {
  println("shell")
  override def start(primaryStage: Stage) {
    primaryStage.setTitle("shell")
    val layout = new Layout    
    val root = layout.gridPane
    val scene = new Scene(root, 300, 300)
    primaryStage.setScene(scene)
    primaryStage.show()
  }
}

object Shell {
  def main(args: Array[String]) {
    Application.launch(classOf[Shell], args: _*)
  }
}
}
