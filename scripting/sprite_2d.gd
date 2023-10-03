extends Sprite2D

var angular_speed = PI

func _process(delta):
	rotation += PI * delta
	position += Vector2(1.0, 1.0)
