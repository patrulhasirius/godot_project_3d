extends Mesh3DRandomShader

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var custom_material = ShaderMaterial.new()
	
	randomize()
	var rand_i = randi()
	print(rand_i)
	custom_material.shader = Shader.new()
	custom_material.shader.code = self.new_shader_code(rand_i, 20)
	
	self.material_override = custom_material
	


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
