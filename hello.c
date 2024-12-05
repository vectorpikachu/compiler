int main() {
  int a = 1, b = 0;
  if (a || b || 0 + 1 || !a || !b || b + a || a * b || b / a || b - a) {
    b = 1;
    a = 0;
  }
  if (a && b && 1 && a == b) {
    return 0;
  }
  int c = 2;
  int d = 5;
  int e = 7;
  if (c < d && d < e || d > a || b < c || a+b < c) {
    return 5;
  }
  const int z = 6;
  if (z == 6 || d == 7 || z) {
    c = c + 5;
    return c;
  }
  return 77;
}
