resource "null_resource" "self" {
  triggers = {
    update = 1
  }
}
