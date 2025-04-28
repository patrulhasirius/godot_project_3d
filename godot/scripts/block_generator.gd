extends Node


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var scene = preload("res://scenes/obelisk.tscn")
	
	var iterable: int = 10
	
	for x in iterable:
		for z in iterable:
			var instance = scene.instantiate()
			instance.scale.y = 10
			instance.scale.x = 3
			instance.position = Vector3(float(x * 6 - (iterable - 1) * 3), 5, float(z * 6 - (iterable - 1) * 3))
			add_child(instance)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
