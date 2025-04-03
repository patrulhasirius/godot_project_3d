extends CSGBox3D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var shader_material := ShaderMaterial.new()
	var shader = Shader.new()
	shader.code = """
		shader_type spatial;
		
		void vertex() {
  			VERTEX.y += cos(VERTEX.x) * sin(VERTEX.z);
		}


		void fragment()
		{
			ALBEDO = vec3(UV, 0.0);
		}
		"""
	shader_material.shader = shader
	self.set("Material/0", shader_material)



# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
