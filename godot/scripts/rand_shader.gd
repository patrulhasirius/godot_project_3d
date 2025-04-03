extends Mesh3DRandomShader

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var shader_material = self.get_active_material(0)
	
	randomize()
	
	shader_material.shader.code = self.new_shader_code(randi(), 20)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
