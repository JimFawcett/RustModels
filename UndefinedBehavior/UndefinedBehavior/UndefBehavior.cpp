// UndefBehavior.cpp

#include <vector>
#include <iostream>

using VecInt = std::vector<int>;

void showVec(const VecInt& v) {
  std::cout << "\n  ";
  for (auto i : v) {    // range-based for
    std::cout << i << " ";
  }
}
void showArray(const int arr[], size_t size) {
  std::cout << "\n  ";
  for (size_t i = 0; i <= size; ++i) {
    std::cout << arr[i] << " ";
  }
}
int main() {

  std::cout << "\n  Demo of Undefined Behavior - dangling reference";
  std::cout << "\n -------------------------------------------------";

  std::vector<int> v;
  v.reserve(3);
  std::cout << "\n  capacity of v = " << v.capacity();
  v.push_back(1);
  v.push_back(2);
  v.push_back(3);
  showVec(v);
  int& r1 = v[1];
  std::cout << "\n  address of v[1] = " << &v[1];
  std::cout << "\n  address of r1 = " << &r1;
  std::cout << "\n  value of r1 = " << r1;
  v.push_back(4);
  showVec(v);
  std::cout << "\n  address of v[1] = " << &v[1];
  std::cout << "\n  address of r1 = " << &r1;
  std::cout << "\n  value of r1 = " << r1;
  std::cout << std::endl;

  std::cout << "\n  Demo of Undefined Behavior - out of bounds index";
  std::cout << "\n --------------------------------------------------";

  int array[3]{ 1, 2, 3 };
  std::cout << "\n  ";
  for (size_t i = 0; i <= 3; ++i) {
    std::cout << array[i] << " ";
  }
  std::cout << std::endl;
}