#include<iostream>

extern "C" {
  void fromCPP(){
    std::cout << "# fromCPP " << std::endl; 
  }
}


