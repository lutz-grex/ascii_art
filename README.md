## 🖼️ ASCII Art CLI

--- 
A simple command-line tool written in Rust to convert images into ASCII art. This tool allows you to resize images, preview them in the terminal, and save the generated ASCII representation to a file.

---
### ✨ Features

- 🖌️ Convert any image into ASCII art
- 📏 Resize images to a specified width while maintaining aspect ratio
- 👀 Preview the ASCII art directly in the terminal
- 💾 Save the ASCII output to a text file
- ⚙️ Configurable via command-line arguments

---

### 🗂️ Project Structure
```bash
src
├── ascii.rs      # ASCII conversion and rendering functions
├── cli.rs        # Command-line argument parsing
├── config.rs     # Configuration struct for the CLI
├── image.rs      # Image loading and resizing utilities
├── main.rs       # Entry point
└── utils.rs      # Optional utility functions
```

---

### 🛠️ Usage
```bash
ascii-art-cli -i <IMAGE_PATH> -o <OUTPUT_PATH> -t <TARGET_WIDTH> -p
```

Arguments:

| Option          | Short | Description                                                     | Default  |
|-----------------|-------|-----------------------------------------------------------------|----------|
| --image         | -i    | Path to the input image file                                     | required |
| --output        | -o    | Path to save the ASCII output                                     | required |
| --target-width  | -t    | Target width for the ASCII art (height is scaled automatically)  | 100      |
| --preview       | -p    | Preview ASCII art in the terminal                                 | false    |

---

### 🎨 Example Output:
```bash
ascii-art-cli -i ./images/example.png -o ascii_output.txt -t 120 -p
```

This will convert example.png into ASCII art, scale it to 120 characters wide, print it in the terminal, and save the output to ascii_output.txt.

```text


                                                -*@@@#=.                                            
                                           =@@@@@@@@@@@@@@@                                         
                                         @@@@@@@@@@@@@@@@@@@@                                       
                                        @@@@@@@@@@@@@@@@@@@@@@%                                     
                                       *@@@@@@@@@@@@@@@@@@@@@@@                                     
                                       @@@@@@@@@@@@@@@@@@@@@@@@*                                    
                                      -@@@@@@@@@@@@@*:=@@@@@@@@@                                    
                                      +@@    +@@@@       @@@@@@@                                    
                                      =@  .   @@@@  @@+  .@@@@@@:                                   
                                      -@  @@= -@@* @@@@   @@@@@@:                                   
                                       @@ =%---------@=  @@@@@@@-                                   
                                       @@%---------------@@@@@@@:                                   
                                      .@-=-------------#--@@@@@@*                                   
                                      #@=-----------#----@@@@@@@@                                   
                                     .@@@ =----------=@.  =@@@@@@@                                  
                                     @@@@   +=----=%       @@@@@@@@                                 
                                    @@@@                    @@@@@@@@=                               
                                   @@@@@                    @@@@@@@@@@                              
                                 @@@@@@.                     @@@@@@@@@@+                            
                               *@@@@@@.                       #@@:@@@@@@@                           
                              @@@@@@@=                         @@@@*@@@@@@.                         
                             @@@@@@@@                           @@@@:@@@@@@@                        
                            @@@@+@@@                             @@@@:@@@@@@@                       
                           @@@#*@@@                               @@@@ @@@@@@=                      
                          @@@@ @@@                                 @@@ +@@@@@@                      
                         @@@@@+@@                                  @@@: @@@@@@@                     
                        -@@@@==@@                                   @@  @@@@@@@.                    
                        @@@@@% @@                                      .@@@@@@@=                    
                        @@@@@@  +                                 @@@@@@@* -@@@:                    
                        @@@@@@@                                 :@@@@@@@@@@@ @@                     
                       @+:::::@@@-                             @::@@@@@@@@@@@@#                     
                      @::::::::+@@@@:                         @::::@@@@@@@:::::*-                   
                 -@@+::::::::::::@@@@@@                       @:::::::::::::::::@:                  
                @::::::::::::::::=@@@@@@                     :@:::::::::::::::::-@                  
               @=::::::::::::::::::@@@@@@-                   +#::::::::::::::::::=@                 
               %#:::::::::::::::::::+@@@@:                   @:::::::::::::::::::::@:               
               :@::::::::::::::::::::=@                     .@::::::::::::::::::::::@.              
               @#::::::::::::::::::::::+*                  .@::::::::::::::::::::::=@               
               @::::::::::::::::::::::::@@@             -@@@%::::::::::::::::::::@@                 
              .@:::::::::::::::::::::::::@@@@@@@@@@@@@@@@@@@:::::::::::::::::@@.                    
               .@*:::::::::::::::::::::::%@@@@@@@@@@@@@@@@@@::::::::::::::-@-                       
                       .-@@@=:::::::::::%@                  #:::::::::::=@                          
                              *@@@%%%@@%                     @@::::::=@+                            
                                                               :#@@+.
```

---

### 📝 Lizenz
MIT License