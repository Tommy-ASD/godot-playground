extends RigidBody2D


var speed = 1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var direction = 0
	if Input.is_action_pressed("ui_right") && Input.is_action_pressed("ui_left"):
		direction = 0
	elif Input.is_action_pressed("ui_right"):
		direction = 1
	elif Input.is_action_pressed("ui_left"):
		direction = -1
	if Input.is_action_pressed("ui_up"):
		set_axis_velocity(Vector2(0, 1))
		
	position += Vector2(speed * direction, 0)

