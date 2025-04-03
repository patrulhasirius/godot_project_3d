extends MeshInstance3D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var shader_material = self.get_active_material(0)
	
	shader_material.shader.code = """
		shader_type spatial;
		render_mode unshaded;

		void fragment()
		{
			ALBEDO = vec3(1.0, 1.0, 0.0);
		}
		"""


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
