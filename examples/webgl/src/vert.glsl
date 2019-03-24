// an attribute will receive data from a buffer
attribute vec4 a_position;

// all shaders have a main function
void main() {
  gl_Position = a_position;
  gl_PointSize = 10.0;
}
